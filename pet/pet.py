class Pet:
    def __init__(self, name, species):
        self.name = name
        self.species = species
        self.mood = "content"  # e.g., content, happy, sad, grumpy
        self.hunger = 50  # Range 0-100, 0 is not hungry
        self.energy = 50  # Range 0-100, 100 is full of energy

    def feed(self):
        self.hunger = max(0, self.hunger - 10)
        self.energy = min(100, self.energy + 5)
        if self.hunger < 20:
            self.mood = "happy"
        elif self.hunger > 80:
            self.mood = "grumpy"
        else:
            self.mood = "content"
        print(f"{self.name} has been fed. Hunger: {self.hunger}/100, Mood: {self.mood}, Energy: {self.energy}/100")

    def play(self):
        self.energy = min(100, self.energy + 20)
        self.hunger = min(100, self.hunger + 10) # Playing should make the pet more hungry
        if self.energy > 80:
            self.mood = "ecstatic" # Changed from happy for more variety
        elif self.energy < 20:
            self.mood = "tired" # Changed from sad for more context
        else:
            self.mood = "content"
        print(f"{self.name} played. Energy: {self.energy}/100, Mood: {self.mood}, Hunger: {self.hunger}/100")

    def status(self):
        print(f"\n--- {self.name}'s Status ---")
        print(f"Species: {self.species}")
        print(f"Mood: {self.mood}")
        print(f"Hunger: {self.hunger}/100")
        print(f"Energy: {self.energy}/100")
        print("-----------------------\n")

    def tick(self):
        self.hunger = min(100, self.hunger + 5)
        self.energy = max(0, self.energy - 5)

        if self.hunger > 70 and self.energy < 30: # Both high hunger and low energy
            self.mood = "very grumpy"
        elif self.hunger > 80 : # Adjusted threshold and condition
            self.mood = "grumpy"
        elif self.energy < 20 : # Adjusted threshold and condition
            self.mood = "exhausted"
        elif self.hunger > 50 or self.energy < 50: # Moderate hunger or energy
            self.mood = "a bit down" # Changed from sad
        elif self.hunger < 20 and self.energy > 80:
            self.mood = "thrilled" # Changed from happy
        else:
            self.mood = "content"
        # The print statement for tick() is intentionally commented out
        # to keep the main loop cleaner, as per the objective's silent tick.
        # print(f"Time passes... {self.name}'s Hunger: {self.hunger}, Energy: {self.energy}, Mood: {self.mood}")
        pass
