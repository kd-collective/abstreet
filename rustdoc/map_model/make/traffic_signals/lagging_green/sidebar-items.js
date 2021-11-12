initSidebarItems({"fn":[["add_stage",""],["four_way_four_stage",""],["is_conflict",""],["make_crosswalk_variable",""],["make_lagging_green_variable",""],["make_signal",""],["make_traffic_signal","Create a traffic signal which has a stage that is: protected straight, protected right, unprotected left, unprotected right on red. Followed by a variable stage that has protected left, unprotected right on red. With a last stage that is all-walk and variable. In some degenerate cases, usually with one or more one-way, this can reduce to stage per road. In some rare cases, usually with an alleyway, oncoming lanes can’t both be protected left turns. In such cases the stage is split into two stages with each having a protected and yeild turn."],["merge_stages",""],["movements",""],["movements_from",""],["multi_way_stages","Build stages. First find roads that are straight across, they are either one-way or two-way. For one-way, add any right or left turns, thus completing the stage. For two-way, two stages will be added. The first stage has protected straight, and right and yield left. The second stage has protected left. Lastly, sometimes oncomming left turns can’t both be protected, if this occurs the 2nd stage will have one direction protected and the other yeild and a 3rd, inverse, stage will be added which has the other direction’s left protected and other yield. Finally, any turns which weren’t assigned, because there are no straights or there are more than just pairs of straight intersections, are assigned a stage. These, too are handled as pairs until one remains, which is handled as a one-way."],["optimize",""],["protected_yield_stage",""],["remove_movement",""],["straight_types",""],["three_way_three_stage",""]]});