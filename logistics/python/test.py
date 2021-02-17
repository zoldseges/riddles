from main.classes import *

# Testing classes

# Trying to create a universe
def create_universe(case):

    # Create 2 cities and connect them to each other.
    # TODO test stops at classes::33. try a debugger.
    if case == 1:
        universe = Universe()
        A = City("A")
        B = City("B")
        C = City("C")
        cities = [A, B, C]
        universe.add_cities(cities)
        A.connect_to_city(cities[0], 1)
        B.connect_to_city(cities[1], 2)
        C.connect_to_city(cities[2], 3)
        for city in cities:
            roads = city.get_roads()
            print("{} is connected has roads {}\n".format(city, roads))
            print("Theese roads are connected to:\n")
            c = 1
            
            for road in roads:
                _cities = road.get_cities()
                print("{}: {}\t{}\n".format(c, _cities[0], _cities[1]))
                c += 1
        # TODO Create assertions
        
    # TODO Add distance for the connections


