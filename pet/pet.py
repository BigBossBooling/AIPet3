class Pet:
    def __init__(self, name, species):
        self.name = name
        self.species = species
        self.mood = "content"
        self.hunger = 50
        self.energy = 50

    def feed(self):
        self.hunger -= 10
        if self.hunger < 0:
            self.hunger = 0
        self.energy += 5
        if self.energy > 100:
            self.energy = 100
        self.mood = "happy"
        print(f"{self.name} has been fed.")

    def play(self):
        self.energy += 20
        if self.energy > 100:
            self.energy = 100
        self.hunger += 10
        if self.hunger > 100:
            self.hunger = 100
        self.mood = "playful"
        print(f"{self.name} played and had fun!")

    def status(self):
        print(f"Name: {self.name}")
        print(f"Species: {self.species}")
        print(f"Mood: {self.mood}")
        print(f"Hunger: {self.hunger}")
        print(f"Energy: {self.energy}")

    def tick(self):
        self.hunger += 5
        if self.hunger > 100:
            self.hunger = 100
        self.energy -= 5
        if self.energy < 0:
            self.energy = 0

        if self.hunger >= 80 or self.energy <= 20:
            self.mood = "sad"
        elif self.hunger >= 60 or self.energy <= 40:
            self.mood = "bored"
        else:
            self.mood = "content"
        print("Time passes...")
