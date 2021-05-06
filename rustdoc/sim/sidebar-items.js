initSidebarItems({"constant":[["BIKE_LENGTH",""],["BUS_LENGTH",""],["FOLLOWING_DISTANCE","At all speeds (including at rest), cars must be at least this far apart, measured from front of one car to the back of the other."],["LIGHT_RAIL_LENGTH",""],["MAX_CAR_LENGTH",""],["MIN_CAR_LENGTH",""],["SPAWN_DIST","When spawning at borders, start the front of the vehicle this far along and gradually appear. Getting too close to EPSILON_DIST can lead to get_draw_car having no geometry at all."]],"enum":[["AgentID",""],["AgentType",""],["AlertHandler",""],["AlertLocation",""],["CarStatus",""],["DelayCause","Why is an agent delayed? If there are multiple reasons, arbitrarily pick one – ie, somebody could be blocked by two conflicting turns."],["DrivingGoal",""],["ExternalTripEndpoint",""],["ParkingSpot",""],["PedCrowdLocation",""],["PersonState",""],["Problem",""],["ScenarioModifier","Transforms an existing Scenario before instantiating it."],["SidewalkPOI","Point of interest, that is"],["TripEndpoint","Specifies where a trip begins or ends."],["TripMode",""],["TripPhaseType",""],["TripPurpose","Lifted from Seattle’s Soundcast model, but seems general enough to use anyhere."],["TripResult",""],["VehicleType",""]],"fn":[["fork_rng","Need to explain this trick – basically keeps consistency between two different simulations when each one might make slightly different sequences of calls to the RNG."]],"mod":[["analytics",""],["cap",""],["events",""],["make","Everything needed to setup a simulation. https://a-b-street.github.io/docs/trafficsim/travel_demand.html for context."],["mechanics",""],["pandemic","An experimental SEIR model by https://github.com/omalaspinas/ glued to the traffic simulation. Transmission may occur when people spend time in shared spaces like buildings, bus stops, and buses."],["recorder",""],["render","Intermediate structures so that sim and game crates don’t have a cyclic dependency."],["router","For vehicles only, not pedestrians. Follows a Path from map_model, but can opportunistically lane-change to avoid a slow lane, can can handle re-planning to look for available parking."],["scheduler",""],["sim",""],["transit",""],["trips",""]],"struct":[["AgentProperties",""],["Analytics","As a simulation runs, different pieces emit Events. The Analytics object listens to these, organizing and storing some information from them. The UI queries Analytics to draw time-series and display statistics."],["BorderSpawnOverTime",""],["CarID",""],["CommutersVehiclesCounts","The number of active vehicles and commuters, broken into different categories."],["CreateCar",""],["CreatePedestrian",""],["DistanceInterval",""],["DrawCarInput",""],["DrawPedCrowdInput",""],["DrawPedestrianInput",""],["ExternalPerson",""],["ExternalTrip",""],["IndividTrip",""],["MapBorders","Lists all border intersections of the map, broken down by mode and whether they support incoming or outgoing traffic."],["OrigPersonID",""],["ParkedCar",""],["PedestrianID",""],["Person",""],["PersonID",""],["PersonSpec",""],["Scenario","A Scenario describes all the input to a simulation. Usually a scenario covers one day."],["ScenarioGenerator",""],["SidewalkSpot",""],["Sim","The Sim ties together all the pieces of the simulation. Its main property is the current time."],["SimFlags","SimFlags specifies a simulation to setup."],["SimOptions","Options controlling the traffic simulation."],["SpawnOverTime",""],["TimeInterval",""],["TripID",""],["TripInfo",""],["TripPhase",""],["UnzoomedAgent",""],["Vehicle",""],["VehicleSpec",""]],"trait":[["SimCallback",""]]});