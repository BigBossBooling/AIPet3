"""
Education Service for managing educational features in the Critter-Craft project.

This service includes functionalities for:
- Enrollment in subjects
- Degree requirements
- Certification processes
"""

from src.pet.advanced_constants import EDUCATION_SUBJECTS, EDUCATION_DEGREES, EDUCATION_CERTIFICATIONS

class EducationService:
    def __init__(self):
        self.enrolled_subjects = {}
        self.completed_degrees = {}
        self.certifications = {}

    def enroll_in_subject(self, pet_id, subject):
        if subject in EDUCATION_SUBJECTS:
            if pet_id not in self.enrolled_subjects:
                self.enrolled_subjects[pet_id] = []
            self.enrolled_subjects[pet_id].append(subject)
            return f"Pet {pet_id} enrolled in {subject}."
        return f"Subject {subject} is not available."

    def complete_degree(self, pet_id, degree):
        if degree in EDUCATION_DEGREES:
            if pet_id not in self.completed_degrees:
                self.completed_degrees[pet_id] = []
            self.completed_degrees[pet_id].append(degree)
            return f"Pet {pet_id} has completed the {degree}."
        return f"Degree {degree} is not recognized."

    def earn_certification(self, pet_id, certification):
        if certification in EDUCATION_CERTIFICATIONS:
            if pet_id not in self.certifications:
                self.certifications[pet_id] = []
            self.certifications[pet_id].append(certification)
            return f"Pet {pet_id} has earned the {certification} certification."
        return f"Certification {certification} is not recognized."

    def get_enrolled_subjects(self, pet_id):
        return self.enrolled_subjects.get(pet_id, [])

    def get_completed_degrees(self, pet_id):
        return self.completed_degrees.get(pet_id, [])

    def get_certifications(self, pet_id):
        return self.certifications.get(pet_id, [])
"""