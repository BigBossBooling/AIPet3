import os
import io # Added for io.BytesIO
from typing import Union, Optional, Dict # Added for type hinting

from .base_generator import BaseGenerator
from weasyprint import HTML
# FontConfiguration might be needed if CSS uses @font-face, import for consistency
from weasyprint.fonts import FontConfiguration

class BankStatementGenerator(BaseGenerator):
    def __init__(self, data: Dict):
        """
        Initializes BankStatementGenerator with statement data.
        Args:
            data (dict): A dictionary containing all bank statement details.
        """
        super().__init__(data)
        # Any specific data preparation for bank statements can be done here if needed.
        # For example, converting date strings in transactions to datetime objects,
        # or ensuring numeric fields are floats/decimals.
        # For now, assume data is pre-processed or template handles formatting.

    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        """
        Generates a bank statement PDF from HTML template and data.

        Args:
            output_path_or_buffer (str or io.BytesIO, optional):
                If a string, path to save PDF.
                If io.BytesIO, PDF is written to this buffer.
                If None, PDF content is returned as bytes.

        Returns:
            bool: True if PDF was saved to path/buffer successfully, False on error.
            bytes: PDF content as bytes if output_path_or_buffer is None and successful.
            None: If an error occurred and returning bytes was intended.
        """
        try:
            # Render HTML using BaseGenerator's utility
            # Template name is 'bank_statement.html'
            rendered_html = self._render_template('bank_statement.html', self.data)

            # Base URL for WeasyPrint to resolve relative paths (e.g., for static files)
            font_config = FontConfiguration() # Default font configuration
            html_doc = HTML(string=rendered_html, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_doc.write_pdf(output_path_or_buffer, font_config=font_config)
                # print(f"Bank statement PDF generated and saved to {output_path_or_buffer}") # Optional
                return True
            elif is_buffer:
                html_doc.write_pdf(target=output_path_or_buffer, font_config=font_config)
                return True # Indicate success
            else: # output_path_or_buffer is None, return bytes
                pdf_bytes = html_doc.write_pdf(font_config=font_config)
                # print("Bank statement PDF generated as bytes.") # Optional
                return pdf_bytes

        except Exception as e:
            print(f"Error generating bank statement PDF: {e}")
            # import traceback
            # traceback.print_exc() # For detailed debugging
            return None if output_path_or_buffer is None else False

# '''
# def generate_bank_statement(statement_data, output_path=None):
#    # ... old code ...

# if __name__ == '__main__':
#    # ... old main block ...
# '''
