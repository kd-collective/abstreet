initSidebarItems({"enum":[["Activity","Different things people might do in the day. Maybe it’s more clear to call this a DestinationType or similar."],["PersonType","It might be useful to classify a CensusPerson into different categories to figure out their Schedule."]],"fn":[["distribute_population_to_homes","Starting from some number of total people living in a polygonal area, randomly distribute them to residential buildings within that area. Returns a list of homes with the number of residents in each."],["generate_scenario","Wires together all the pieces, so you can just hand this any map, and it’ll automatically find appropriate census data, and use it to produce a Scenario."]],"mod":[["activities",""],["distribute_people",""],["import_census",""],["make_person",""],["od","This is an alternative pipeline for generating a Scenario, starting from origin-destination data (also called desire lines), which gives a count of commuters between two zones, breaking down by mode."]],"struct":[["CensusArea","Represents aggregate demographic data for some part of a city. These could be census tracts or blocks, depending what data we find. All of the areas should roughly partition the map – we probably don’t need to guarantee we cover every single building, but we definitely shouldn’t have two overlapping areas."],["CensusPerson","Demographic information for a single person"],["Config","Any arbitrarily chosen parameters needed should be put here, so they can be controlled from the UI or tuned for different cities."],["Schedule","A single person’s daily schedule. It’s assumed that someone always starts at home. And for most people, the last entry should probably be Activity::Home."]]});