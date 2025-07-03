"""
This file manages educational features, including subject offerings, degree requirements, and certification processes for pets.
"""

from advanced_constants import EDUCATION_SUBJECTS, EDUCATION_DEGREES, EDUCATION_CERTIFICATIONS

class EducationManager:
    def __init__(self):
        self.enrolled_subjects = {}
        self.completed_degrees = []
        self.certifications = []

    def enroll_in_subject(self, pet, subject):
        if subject in EDUCATION_SUBJECTS:
            if pet not in self.enrolled_subjects:
                self.enrolled_subjects[pet] = []
            self.enrolled_subjects[pet].append(subject)
            return f"{pet} has been enrolled in {subject}."
        return "Subject not available."

    def complete_degree(self, pet, degree):
        if degree in EDUCATION_DEGREES and degree not in self.completed_degrees:
            self.completed_degrees.append(degree)
            return f"{pet} has completed the {degree}."
        return "Degree not available or already completed."

    def earn_certification(self, pet, certification):
        if certification in EDUCATION_CERTIFICATIONS and certification not in self.certifications:
            self.certifications.append(certification)
            return f"{pet} has earned the {certification} certification."
        return "Certification not available or already earned."

    def get_enrolled_subjects(self, pet):
        return self.enrolled_subjects.get(pet, [])

    def get_completed_degrees(self):
        return self.completed_degrees

    def get_certifications(self):
        return self.certifications
