from ragger.conftest import configuration

###########################
### CONFIGURATION START ###
###########################

# You can configure optional parameters by overriding the value of ragger.configuration.OPTIONAL_CONFIGURATION
# Please refer to ragger/conftest/configuration.py for their descriptions and accepted values
configuration.OPTIONAL.BACKEND_SCOPE = "function"
configuration.OPTIONAL.APP_DIR = "./workdir/app-near"

#########################
### CONFIGURATION END ###
#########################



# Pull all features from the base ragger conftest using the overridden configuration
pytest_plugins = ("ragger.conftest.base_conftest", )
