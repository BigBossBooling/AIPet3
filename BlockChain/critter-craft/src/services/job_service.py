from src.pet.advanced_constants import JOB_TYPES

class JobService:
    def __init__(self):
        self.jobs = JOB_TYPES

    def assign_job(self, pet, job_type):
        if job_type in self.jobs:
            job = self.jobs[job_type]
            if self._meets_requirements(pet, job['requirements']):
                pet.job = job_type
                pet.salary = job['base_salary']
                pet.experience = 0
                return f"{pet.name} has been assigned the job of {job['display_name']}."
            else:
                return f"{pet.name} does not meet the requirements for the job of {job['display_name']}."
        else:
            return "Job type not found."

    def _meets_requirements(self, pet, requirements):
        for stat, min_value in requirements['min_stats'].items():
            if getattr(pet, stat, 0) < min_value:
                return False
        return pet.age >= requirements['min_age']

    def calculate_salary(self, pet):
        if hasattr(pet, 'job'):
            job = self.jobs[pet.job]
            return job['base_salary']
        return 0

    def gain_experience(self, pet, amount):
        if hasattr(pet, 'job'):
            pet.experience += amount
            return f"{pet.name} gained {amount} experience in their job."
        return "Pet does not have a job assigned."
