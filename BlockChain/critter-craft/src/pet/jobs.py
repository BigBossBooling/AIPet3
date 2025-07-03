"""
Job-related classes and functions for managing job assignments and statistics for pets.
"""

class Job:
    def __init__(self, job_type, pet):
        self.job_type = job_type
        self.pet = pet
        self.salary = self.get_salary()
        self.experience = 0

    def get_salary(self):
        return JOB_TYPES[self.job_type]["base_salary"]

    def work(self):
        self.experience += JOB_TYPES[self.job_type]["exp_per_work"]
        return self.salary

    def is_qualified(self):
        requirements = JOB_TYPES[self.job_type]["requirements"]
        return (self.pet.age >= requirements["min_age"] and
                all(getattr(self.pet, stat) >= value for stat, value in requirements["min_stats"].items()))

class JobManager:
    def __init__(self):
        self.jobs = []

    def assign_job(self, job_type, pet):
        if job_type in JOB_TYPES and pet.is_qualified_for_job(job_type):
            job = Job(job_type, pet)
            self.jobs.append(job)
            return job
        return None

    def calculate_total_salary(self):
        return sum(job.salary for job in self.jobs)

    def get_job_statistics(self):
        return {
            "total_jobs": len(self.jobs),
            "total_salary": self.calculate_total_salary(),
            "average_experience": (sum(job.experience for job in self.jobs) / len(self.jobs)) if self.jobs else 0
        }