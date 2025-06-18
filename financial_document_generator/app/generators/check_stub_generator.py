import os
import io # Added for io.BytesIO
from typing import Union, Optional, Dict # Added for type hinting

# Jinja2 and WeasyPrint imports will be effectively handled by BaseGenerator or direct use
from .base_generator import BaseGenerator
from weasyprint import HTML
from weasyprint.fonts import FontConfiguration # Required for @font-face in CSS if any

class CheckStubGenerator(BaseGenerator):
    def __init__(self, data: Dict):
        super().__init__(data)
        # Data for check stub is generally straightforward, less pre-processing needed here
        # compared to CheckGenerator which handles fonts/logos.
        # If any specific stub data prep is needed, it can be done here.

    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        """
        Generates a check stub PDF from HTML template and data.

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
            # The template 'check_stub.html' should be directly under the 'templates' folder.
            rendered_html = self._render_template('check_stub.html', self.data)

            # Base URL for WeasyPrint
            # Important for resolving any relative paths in the HTML (e.g., static files)
            # if they were not handled by Jinja's url_for (which they won't be here).
            font_config = FontConfiguration() # Default font configuration
            html_doc = HTML(string=rendered_html, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_doc.write_pdf(output_path_or_buffer, font_config=font_config)
                # print(f"Check stub PDF generated and saved to {output_path_or_buffer}") # Optional: for debugging
                return True
            elif is_buffer:
                html_doc.write_pdf(target=output_path_or_buffer, font_config=font_config)
                # Buffer is modified in-place
                return True # Indicate success
            else: # output_path_or_buffer is None, return bytes
                pdf_bytes = html_doc.write_pdf(font_config=font_config)
                # print("Check stub PDF generated as bytes.") # Optional: for debugging
                return pdf_bytes

        except Exception as e:
            print(f"Error generating check stub PDF: {e}")
            # import traceback
            # traceback.print_exc() # For detailed debugging
            return None if output_path_or_buffer is None else False

# '''
# def generate_check_stub(data: dict, output_path: str = None):
#     # ... old function ...
# if __name__ == '__main__':
#     # ... old main block ...
# '''
