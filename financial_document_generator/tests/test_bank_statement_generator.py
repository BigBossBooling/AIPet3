import unittest
from ..app.bank_statement_generator import generate_bank_statement

class TestBankStatementGenerator(unittest.TestCase):

    def test_generate_bank_statement_smoke(self):
        # Smoke test to ensure the function runs without crashing
        sample_data = {"account_holder": "John Doe", "statement_period": "2023-10"}
        try:
            generate_bank_statement(sample_data)
            ran_successfully = True
        except Exception as e:
            print(f"Test failed: {e}")
            ran_successfully = False
        self.assertTrue(ran_successfully, "generate_bank_statement should run without errors.")

if __name__ == '__main__':
    unittest.main()
