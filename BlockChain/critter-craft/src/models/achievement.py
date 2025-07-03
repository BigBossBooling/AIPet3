class Achievement:
    def __init__(self, name, description, required_progress, points):
        self.name = name
        self.description = description
        self.required_progress = required_progress
        self.points = points
        self.is_achieved = False

    def update_progress(self, progress):
        if progress >= self.required_progress:
            self.is_achieved = True

    def __repr__(self):
        return f"Achievement(name={self.name}, description={self.description}, points={self.points}, is_achieved={self.is_achieved})"