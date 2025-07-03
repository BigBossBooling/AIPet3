class Pet:
    def __init__(self, name, age, stats=None):
        self.name = name
        self.age = age
        self.stats = stats if stats is not None else {
            "iq": 0,
            "charisma": 0,
            "energy": 0,
            "happiness": 0,
            "social": 0,
            "cleanliness": 0
        }
        self.jobs = []
        self.achievements = []
        self.evolution_stage = None
        self.dna_traits = {}

    def add_job(self, job):
        self.jobs.append(job)

    def remove_job(self, job):
        if job in self.jobs:
            self.jobs.remove(job)

    def gain_experience(self, amount):
        # Logic to gain experience and level up
        pass

    def evolve(self, evolution_path):
        # Logic to evolve the pet based on the evolution path
        pass

    def update_stats(self, stat_changes):
        for stat, change in stat_changes.items():
            if stat in self.stats:
                self.stats[stat] += change

    def add_achievement(self, achievement):
        if achievement not in self.achievements:
            self.achievements.append(achievement)

    def set_dna_traits(self, traits):
        self.dna_traits = traits

    def __str__(self):
        return f"{self.name}, Age: {self.age}, Stats: {self.stats}, Jobs: {self.jobs}, Achievements: {self.achievements}"
