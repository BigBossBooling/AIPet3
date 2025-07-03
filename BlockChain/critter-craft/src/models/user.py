class User:
    def __init__(self, username):
        self.username = username
        self.pets = []

    def add_pet(self, pet):
        self.pets.append(pet)

    def remove_pet(self, pet):
        if pet in self.pets:
            self.pets.remove(pet)

    def get_pets(self):
        return self.pets

    def __str__(self):
        return f"User: {self.username}, Pets: {len(self.pets)}"