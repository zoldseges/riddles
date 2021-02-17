class Universe():

    def __init__(self):
        self.cities = []
        # TODO roads?

    def add_city(self, name):
        self.cities.append(City(name))

    def add_cities(self, cities):
        for city in cities:
            if isinstance(city, City):
                cities.append(city)
            else:
                raise Exception("Dude, add CITIES! not add whatever you like!")

class Packet():
    
    def __init__(self, _from, _to):
        self._from = _from
        self._to = _to


class Road():
    
    def __init__(self):
        self._from = None
        self._to = None
        self.length = None

    def set_connection(self, city):
        if isinstance(city, City):
            print("test 0 stops here")
            if self._from == None:
                self._from = city
            elif self._to == None:
                self._to = city
            else:
                raise Exception("Road is already connected")
        else:
            raise Exception("You have to connect road to a city (argument is not a city)")

    def set_length(self, length):
        self.length = length

    def get_cities(self):
        """
        Return tuple of cities the road is connected to.
        """
        return (self._from, self._to)

    def get_length(self):
        return self.length
        
class City():

    def __init__(self, name):
        self.name = name
        self.packets = []
        self.roads = []

    def connect_to_city(self, city, dist):
        """
        Update both cities roads lists by the same road.
        """
        if isinstance(city, City):
            road = Road()
            road.set_length(dist)
            road.set_connection(self)
            road.set_connection(city)
            self.roads.append(road)
            city.add_road(road)
        else:
            raise Exception("What you're trying to connect is not a city!")

    def add_road(self, road, dist):
        if isinstance(road, Road):
            self.roads.append(road)
        else: raise Exception("It's not a road what you're trying to add")
        
    def get_roads(self):
        return self.roads
        
    def add_packet(self):
        self.packets.append(Packet())
        
class Lorry():

    def __init__(self):
        self.city = City()
        self.packets = []
    
    def pick_up(self, packet):
        pass
