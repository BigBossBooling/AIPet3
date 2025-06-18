import unittest
from ..app.generators.income_statement_generator import generate_income_statement

class TestIncomeStatementGenerator(unittest.TestCase):

    def test_generate_income_statement_smoke(self):
        # Smoke test to ensure the function runs without crashing
        sample_data = {"company_name": "Tech Corp", "period_ending": "2023-12-31"}
        try:
            generate_income_statement(sample_data)
            ran_successfully = True
        except Exception as e:
            print(f"Test failed: {e}")
            ran_successfully = False
        self.assertTrue(ran_successfully, "generate_income_statement should run without errors.")

if __name__ == '__main__':
    unittest.main()
