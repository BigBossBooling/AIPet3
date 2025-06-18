import os

class Config:
    SECRET_KEY = os.environ.get('SECRET_KEY') or 'a-very-secret-and-hard-to-guess-key-for-dev'
    DEBUG = False
    TESTING = False

class DevelopmentConfig(Config):
    DEBUG = True

class TestingConfig(Config):
    TESTING = True
    DEBUG = True # Often helpful for tests
    SECRET_KEY = 'test-secret-key' # Consistent key for tests

class ProductionConfig(Config):
    # SECRET_KEY must be set from an environment variable in production
    SECRET_KEY = os.environ.get('SECRET_KEY')
    if not SECRET_KEY:
        # In a real app, you might raise an error or have a more complex fallback
        print("Warning: SECRET_KEY is not set for production!")
        SECRET_KEY = 'fallback-prod-key-should-be-env-var'


config_by_name = dict(
    development=DevelopmentConfig,
    testing=TestingConfig,
    production=ProductionConfig,
    default=DevelopmentConfig
)

def get_config_name():
    return os.getenv('FLASK_CONFIG', 'default')
