import os
from jinja2 import Environment, FileSystemLoader

class BaseGenerator:
    def __init__(self, data: dict):
        """
        Initializes the BaseGenerator with data.

        Args:
            data (dict): The data dictionary for the document.
        """
        self.data = data

    def generate_pdf(self, output_path_or_buffer=None):
        """
        Generates a PDF document. This method should be implemented by subclasses.

        Args:
            output_path_or_buffer (str or io.BytesIO, optional):
                If a string, it's the path to save the PDF file.
                If an io.BytesIO object, the PDF is written to this buffer.
                If None, the PDF content should be returned as bytes.

        Returns:
            bool: True if PDF was saved to a file path successfully.
            bytes: PDF content as bytes if output_path_or_buffer is None.
            None: If an error occurred or if writing to a provided buffer. (Subclass defines exact return for buffer case)

        Raises:
            NotImplementedError: If the subclass does not implement this method.
        """
        raise NotImplementedError("Subclasses must implement the generate_pdf method.")

    def _render_template(self, template_name: str, context: dict = None) -> str:
        """
        Renders an HTML template using Jinja2.

        Args:
            template_name (str): The name of the template file (e.g., 'form.html').
            context (dict, optional): The context dictionary to pass to the template.
                                      If None, self.data is used.

        Returns:
            str: The rendered HTML content.

        Raises:
            Exception: Can raise various exceptions related to template loading or rendering.
        """
        if context is None:
            context = self.data

        # Calculate path to templates directory, assuming it's ../../templates from this file
        # financial_document_generator/app/generators/base_generator.py -> financial_document_generator/templates
        script_path = os.path.abspath(__file__) # -> /app/financial_document_generator/app/generators/base_generator.py
        generators_dir = os.path.dirname(script_path) # -> .../app/generators
        app_dir = os.path.dirname(generators_dir) # -> .../app
        project_root_dir = os.path.dirname(app_dir) # -> .../financial_document_generator (project root)

        templates_dir = os.path.join(project_root_dir, 'templates')

        env = Environment(loader=FileSystemLoader(templates_dir))
        template = env.get_template(template_name)

        return template.render(context)

    def _get_project_root(self) -> str:
        """Helper to get the project root directory."""
        script_path = os.path.abspath(__file__)
        generators_dir = os.path.dirname(script_path)
        app_dir = os.path.dirname(generators_dir)
        project_root_dir = os.path.dirname(app_dir)
        return project_root_dir
