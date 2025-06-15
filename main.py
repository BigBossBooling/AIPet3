from pet.pet import Pet

def main():
    pet_name = input("Enter your pet's name: ")
    pet_species = input("Enter your pet's species: ")
    player_pet = Pet(pet_name, pet_species)

    while True:
        print("\nWhat would you like to do?")
        print("1. Feed")
        print("2. Play")
        print("3. Check status")
        print("4. Do nothing")
        print("5. Exit")

        choice = input("Enter your choice (1-5): ")

        if choice == '1':
            player_pet.feed()
        elif choice == '2':
            player_pet.play()
        elif choice == '3':
            player_pet.status()
        elif choice == '4':
            print(f"{player_pet.name} did nothing.")
        elif choice == '5':
            print("Exiting game. Goodbye!")
            break
        else:
            print("Invalid choice. Please enter a number between 1 and 5.")

        player_pet.tick()

if __name__ == "__main__":
    main()
