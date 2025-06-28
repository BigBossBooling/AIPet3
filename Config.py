from pet.pet import Pet

def main():
    print("Welcome to Virtual Pet!")
    pet_name = input("What would you like to name your pet? ")
    pet_species = input(f"What species is {pet_name}? (e.g., cat, dog, dragon) ")

    player_pet = Pet(name=pet_name, species=pet_species)
    print(f"\nCongratulations! You've adopted {player_pet.name} the {player_pet.species}.")
    player_pet.status()

    while True:
        print("\nWhat would you like to do?")
        print("1. Feed")
        print("2. Play")
        print("3. Check status")
        print("4. Do nothing (let time pass)")
        print("5. Exit")

        choice = input("Enter your choice (1-5): ")

        action_taken = True # Assume an action will lead to a tick by default
        if choice == '1':
            player_pet.feed()
        elif choice == '2':
            player_pet.play()
        elif choice == '3':
            player_pet.status()
            action_taken = False # Checking status doesn't pass time
        elif choice == '4':
            print(f"You decided to do nothing with {player_pet.name}.")
            # Tick will be called, action_taken remains true
        elif choice == '5':
            print(f"Goodbye! Hope you and {player_pet.name} had fun!")
            break
        else:
            print("Invalid choice. Please enter a number between 1 and 5.")
            action_taken = False # Invalid choice doesn't pass time

        # Call tick if an action was taken that should result in time passing
        if action_taken:
             print("A moment passes...")
             player_pet.tick()
             # Display a brief update after tick
             print(f"{player_pet.name}'s mood is now {player_pet.mood}, Hunger: {player_pet.hunger}/100, Energy: {player_pet.energy}/100.")

if __name__ == "__main__":
    main()
