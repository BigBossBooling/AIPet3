import unittest
from ..app.generators.earning_statement_generator import generate_earning_statement

class TestEarningStatementGenerator(unittest.TestCase):

    def test_generate_earning_statement_smoke(self):
        # Smoke test to ensure the function runs without crashing
        sample_data = {"employee_name": "John Smith", "pay_period": "2023-11-01 to 2023-11-15"}
        try:
            generate_earning_statement(sample_data)
            ran_successfully = True
        except Exception as e:
            print(f"Test failed: {e}")
            ran_successfully = False
        self.assertTrue(ran_successfully, "generate_earning_statement should run without errors.")

if __name__ == '__main__':
    unittest.main()
