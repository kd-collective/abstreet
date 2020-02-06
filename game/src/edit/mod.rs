mod lanes;
mod stop_signs;
mod traffic_signals;

pub use self::lanes::LaneEditor;
pub use self::stop_signs::StopSignEditor;
pub use self::traffic_signals::TrafficSignalEditor;
use crate::common::{tool_panel, Colorer, CommonState, Overlays, Warping};
use crate::debug::DebugMode;
use crate::game::{msg, State, Transition, WizardState};
use crate::helpers::ID;
use crate::managed::{WrappedComposite, WrappedOutcome};
use crate::render::{DrawIntersection, DrawLane, DrawRoad};
use crate::sandbox::{GameplayMode, SandboxMode};
use crate::ui::UI;
use abstutil::Timer;
use ezgui::{hotkey, lctrl, Choice, EventCtx, GfxCtx, Key, Line, ModalMenu, Text, WrappedWizard};
use map_model::{
    connectivity, EditCmd, IntersectionID, LaneID, LaneType, MapEdits, PathConstraints,
};
use sim::Sim;
use std::collections::BTreeSet;

pub struct EditMode {
    tool_panel: WrappedComposite,
    menu: ModalMenu,

    // Retained state from the SandboxMode that spawned us
    mode: GameplayMode,
    pub suspended_sim: Sim,

    once: bool,
}

impl EditMode {
    pub fn new(ctx: &mut EventCtx, ui: &mut UI, mode: GameplayMode) -> EditMode {
        let suspended_sim = ui.primary.clear_sim();
        EditMode {
            tool_panel: tool_panel(ctx),
            menu: ModalMenu::new(
                "Map Edit Mode",
                vec![
                    (hotkey(Key::S), "save edits"),
                    (hotkey(Key::L), "load different edits"),
                    // TODO Support redo. Bit harder here to reset the redo_stack when the edits
                    // change, because nested other places modify it too.
                    (lctrl(Key::Z), "undo"),
                    (hotkey(Key::Escape), "finish editing"),
                ],
                ctx,
            ),
            mode,
            suspended_sim,
            once: true,
        }
    }

    fn quit(&self, ctx: &mut EventCtx, ui: &mut UI) -> Transition {
        ctx.loading_screen("apply edits", |ctx, mut timer| {
            ui.overlay = Overlays::Inactive;
            ui.primary
                .map
                .recalculate_pathfinding_after_edits(&mut timer);
            // Parking state might've changed
            ui.primary.clear_sim();
            Transition::PopThenReplace(Box::new(SandboxMode::new(ctx, ui, self.mode.clone())))
        })
    }
}

impl State for EditMode {
    fn event(&mut self, ctx: &mut EventCtx, ui: &mut UI) -> Transition {
        // Can't do this in the constructor, because SandboxMode's on_destroy clears out Overlays
        if self.once {
            self.once = false;
            // apply_map_edits will do the job later
            ui.overlay = Overlays::map_edits(ctx, ui);

            let edits = ui.primary.map.get_edits();
            let mut txt = Text::new();
            txt.add(Line(format!("Edits: {}", edits.edits_name)));
            if edits.dirty {
                txt.append(Line("*"));
            }
            self.menu.set_info(ctx, txt);
        }

        self.menu.event(ctx);

        ctx.canvas_movement();
        // Restrict what can be selected.
        if ctx.redo_mouseover() {
            ui.recalculate_current_selection(ctx);
            if let Some(ID::Lane(l)) = ui.primary.current_selection {
                if !can_edit_lane(&self.mode, l, ui) {
                    ui.primary.current_selection = None;
                }
            } else if let Some(ID::Intersection(_)) = ui.primary.current_selection {
            } else {
                ui.primary.current_selection = None;
            }
        }

        if ui.opts.dev && ctx.input.new_was_pressed(lctrl(Key::D).unwrap()) {
            return Transition::Push(Box::new(DebugMode::new(ctx)));
        }

        if ui.primary.map.get_edits().dirty && self.menu.action("save edits") {
            return Transition::Push(WizardState::new(Box::new(|wiz, ctx, ui| {
                save_edits(&mut wiz.wrap(ctx), ui)?;
                Some(Transition::Pop)
            })));
        } else if self.menu.action("load different edits") {
            return Transition::Push(make_load_edits(self.mode.clone()));
        } else if self.menu.action("finish editing") {
            return self.quit(ctx, ui);
        }

        if let Some(ID::Intersection(id)) = ui.primary.current_selection {
            if ui.primary.map.maybe_get_stop_sign(id).is_some()
                && self.mode.can_edit_stop_signs()
                && ui.per_obj.left_click(ctx, "edit stop signs")
            {
                return Transition::Push(Box::new(StopSignEditor::new(id, ctx, ui)));
            }
            if ui.primary.map.maybe_get_traffic_signal(id).is_some()
                && ui.per_obj.left_click(ctx, "edit traffic signal")
            {
                return Transition::Push(Box::new(TrafficSignalEditor::new(
                    id,
                    ctx,
                    ui,
                    self.suspended_sim.clone(),
                )));
            }
            if ui.primary.map.get_i(id).is_closed()
                && ui.per_obj.left_click(ctx, "re-open closed intersection")
            {
                let mut edits = ui.primary.map.get_edits().clone();
                edits
                    .commands
                    .push(EditCmd::UncloseIntersection(id, edits.original_it(id)));
                apply_map_edits(ctx, ui, edits);
            }
        }
        if let Some(ID::Lane(l)) = ui.primary.current_selection {
            if ui.per_obj.left_click(ctx, "edit lane") {
                return Transition::Push(Box::new(LaneEditor::new(l, ctx, ui)));
            }
        }

        if !ui.primary.map.get_edits().commands.is_empty() && self.menu.action("undo") {
            let mut edits = ui.primary.map.get_edits().clone();
            let id = match edits.commands.pop().unwrap() {
                EditCmd::ChangeLaneType { id, .. } => ID::Lane(id),
                EditCmd::ReverseLane { l, .. } => ID::Lane(l),
                EditCmd::ChangeStopSign(ss) => ID::Intersection(ss.id),
                EditCmd::ChangeTrafficSignal(ss) => ID::Intersection(ss.id),
                EditCmd::CloseIntersection { id, .. } => ID::Intersection(id),
                EditCmd::UncloseIntersection(id, _) => ID::Intersection(id),
            };
            apply_map_edits(ctx, ui, edits);
            return Transition::Push(Warping::new(
                ctx,
                id.canonical_point(&ui.primary).unwrap(),
                None,
                Some(id),
                &mut ui.primary,
            ));
        }

        match self.tool_panel.event(ctx, ui) {
            Some(WrappedOutcome::Transition(t)) => t,
            Some(WrappedOutcome::Clicked(x)) => match x.as_ref() {
                "back" => self.quit(ctx, ui),
                _ => unreachable!(),
            },
            None => Transition::Keep,
        }
    }

    fn draw(&self, g: &mut GfxCtx, ui: &UI) {
        // TODO Maybe this should be part of ui.draw
        // TODO This has an X button, but we never call update and allow it to be changed. Should
        // just omit the button.
        ui.overlay.draw(g);

        self.tool_panel.draw(g);
        self.menu.draw(g);
        CommonState::draw_osd(g, ui, &ui.primary.current_selection);
    }
}

pub fn save_edits(wizard: &mut WrappedWizard, ui: &mut UI) -> Option<()> {
    let map = &mut ui.primary.map;

    let rename = if map.get_edits().edits_name == "no_edits" {
        Some(wizard.input_something(
            "Name these map edits",
            None,
            Box::new(|l| {
                if l.contains("/") || l == "no_edits" || l == "" {
                    None
                } else {
                    Some(l)
                }
            }),
        )?)
    } else {
        None
    };

    // TODO Do it this weird way to avoid saving edits on every event. :P
    // TODO Do some kind of versioning? Don't ask this if the file doesn't exist yet?
    let save = "save edits";
    let cancel = "cancel";
    if wizard
        .choose_string("Overwrite edits?", || vec![save, cancel])?
        .as_str()
        == save
    {
        if let Some(name) = rename {
            let mut edits = map.get_edits().clone();
            edits.edits_name = name;
            map.apply_edits(edits, &mut Timer::new("name map edits"));
        }
        map.save_edits();
    }
    Some(())
}

fn make_load_edits(mode: GameplayMode) -> Box<dyn State> {
    WizardState::new(Box::new(move |wiz, ctx, ui| {
        let mut wizard = wiz.wrap(ctx);

        if ui.primary.map.get_edits().dirty {
            let save = "save edits";
            let discard = "discard";
            if wizard
                .choose_string("Save current edits first?", || vec![save, discard])?
                .as_str()
                == save
            {
                save_edits(&mut wizard, ui)?;
                wizard.reset();
            }
        }

        // TODO Exclude current
        let map_name = ui.primary.map.get_name().to_string();
        let (_, new_edits) = wizard.choose("Load which map edits?", || {
            let mut list = Choice::from(
                abstutil::load_all_objects(abstutil::path_all_edits(&map_name))
                    .into_iter()
                    .filter(|(_, edits)| mode.allows(edits))
                    .collect(),
            );
            list.push(Choice::new("no_edits", MapEdits::new(map_name.clone())));
            list
        })?;
        apply_map_edits(ctx, ui, new_edits);
        ui.primary.map.mark_edits_fresh();
        Some(Transition::Pop)
    }))
}

pub fn apply_map_edits(ctx: &mut EventCtx, ui: &mut UI, mut edits: MapEdits) {
    edits.dirty = true;
    let mut timer = Timer::new("apply map edits");

    let (lanes_changed, roads_changed, turns_deleted, turns_added, mut modified_intersections) =
        ui.primary.map.apply_edits(edits, &mut timer);

    for l in lanes_changed {
        ui.primary.draw_map.lanes[l.0] = DrawLane::new(
            ui.primary.map.get_l(l),
            &ui.primary.map,
            ui.primary.current_flags.draw_lane_markings,
            &ui.cs,
            &mut timer,
        )
        .finish(ctx.prerender);
    }
    for r in roads_changed {
        ui.primary.draw_map.roads[r.0] = DrawRoad::new(
            ui.primary.map.get_r(r),
            &ui.primary.map,
            &ui.cs,
            ctx.prerender,
        );
    }

    let mut lanes_of_modified_turns: BTreeSet<LaneID> = BTreeSet::new();
    for t in turns_deleted {
        lanes_of_modified_turns.insert(t.src);
        modified_intersections.insert(t.parent);
    }
    for t in &turns_added {
        lanes_of_modified_turns.insert(t.src);
        modified_intersections.insert(t.parent);
    }

    for i in modified_intersections {
        ui.primary.draw_map.intersections[i.0] = DrawIntersection::new(
            ui.primary.map.get_i(i),
            &ui.primary.map,
            &ui.cs,
            ctx.prerender,
            &mut timer,
        );
    }

    if let Overlays::Edits(_) = ui.overlay {
        ui.overlay = Overlays::map_edits(ctx, ui);
    }
}

pub fn can_edit_lane(mode: &GameplayMode, l: LaneID, ui: &UI) -> bool {
    mode.can_edit_lanes()
        && !ui.primary.map.get_l(l).is_sidewalk()
        && ui.primary.map.get_l(l).lane_type != LaneType::SharedLeftTurn
}

pub fn close_intersection(ctx: &mut EventCtx, ui: &mut UI, i: IntersectionID) -> Transition {
    let it = ui.primary.map.get_i(i).intersection_type;

    let mut edits = ui.primary.map.get_edits().clone();
    edits
        .commands
        .push(EditCmd::CloseIntersection { id: i, orig_it: it });
    apply_map_edits(ctx, ui, edits);

    let (_, disconnected) = connectivity::find_scc(&ui.primary.map, PathConstraints::Pedestrian);
    if disconnected.is_empty() {
        // Success! Quit the stop sign / signal editor.
        return Transition::Pop;
    }

    let mut edits = ui.primary.map.get_edits().clone();
    edits.commands.pop();
    apply_map_edits(ctx, ui, edits);

    let mut err_state = msg(
        "Error",
        vec![format!("{} sidewalks disconnected", disconnected.len())],
    );

    let color = ui.cs.get("unreachable lane");
    let mut c = Colorer::new(Text::new(), vec![("", color)]);
    for l in disconnected {
        c.add_l(l, color, &ui.primary.map);
    }

    err_state.downcast_mut::<WizardState>().unwrap().also_draw = Some(c.build_zoomed(ctx, ui));
    Transition::Push(err_state)
}
