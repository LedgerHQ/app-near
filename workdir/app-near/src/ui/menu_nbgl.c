#ifdef HAVE_NBGL

#include "menu.h"
#include "globals.h"
#include "nbgl_use_case.h"


//  ----------------------------------------------------------- 
//  --------------------- SETTINGS MENU -----------------------
//  ----------------------------------------------------------- 
static const char* const INFO_TYPES[] = {"Version", "Developer"};
static const char* const INFO_CONTENTS[] = {APPVERSION, "NEAR Protocol"};

#define MAX_STRING_LENGTH 100

static char settings_title[MAX_STRING_LENGTH] = {0};

enum {
    BLIND_SIGN_SWITCH_IDX = 0,
    SWITCHES_TOTAL,
};
static nbgl_layoutSwitch_t G_switches[SWITCHES_TOTAL];

enum {
    BLIND_SIGN_SWITCH_TOKEN = FIRST_USER_TOKEN,
};

#define SETTINGS_PAGES_TOTAL 2

static bool settings_nav_callback(uint8_t page, nbgl_pageContent_t *content)
{
  if (page == 0)
  {
    content->type = INFOS_LIST;
    content->infosList.nbInfos = 2;
    content->infosList.infoTypes = INFO_TYPES;
    content->infosList.infoContents = INFO_CONTENTS;
    return true;
  }
  else if (page == 1)
  {

    G_switches[BLIND_SIGN_SWITCH_IDX].initState = blind_sign_enabled;

    content->type = SWITCHES_LIST;
    content->switchesList.nbSwitches = SWITCHES_TOTAL;
    content->switchesList.switches = G_switches;
    return true;
  }
  else
  {
    return false;
  }
}

// callback for setting warning choice
static void review_warning_choice(bool confirm)
{
  uint8_t switch_value;
  if (confirm)
  {
    // toggle the switch value
    switch_value = !blind_sign_enabled;
    // store the new setting value in NVM
    nvm_write((void *)&N_storage.blind_sign_enabled, &switch_value, sizeof(switch_value));
    blind_sign_enabled = switch_value;
  }

  // return to the settings menu
  ui_menu_settings();
}

static void settings_controls_callback(int token, uint8_t index)
{
  UNUSED(index);
  switch (token)
  {
  case BLIND_SIGN_SWITCH_TOKEN:
    if (!blind_sign_enabled)
    {
      // Display the warning message and ask the user to confirm
      nbgl_useCaseChoice(&C_warning64px,
                         "Blind Sign",
                         "Are you sure to\nallow blind signing\ntransactions?",
                         "I understand, confirm",
                         "Cancel",
                         review_warning_choice);
    }
    else
    {
      uint8_t switch_value;
      // toggle the switch value
      switch_value = !blind_sign_enabled;
      // store the new setting value in NVM
      nvm_write((void *)&N_storage.blind_sign_enabled, &switch_value, sizeof(switch_value));
      blind_sign_enabled = switch_value;
    }
    break;
  default:
    PRINTF("Unreachable in `settings_controls_callback`\n");
    break;
  }
}
// info menu definition
void ui_menu_settings(void)
{
  #define INIT_INFO_PAGE (0)

  G_switches[BLIND_SIGN_SWITCH_IDX].text = "Blind Sign";
  G_switches[BLIND_SIGN_SWITCH_IDX].subText = "Enable blind signing";
  G_switches[BLIND_SIGN_SWITCH_IDX].token = BLIND_SIGN_SWITCH_TOKEN;
  G_switches[BLIND_SIGN_SWITCH_IDX].tuneId = TUNE_TAP_CASUAL;

  strlcpy(settings_title, APPNAME, MAX_STRING_LENGTH);
  nbgl_useCaseSettings(settings_title, INIT_INFO_PAGE, SETTINGS_PAGES_TOTAL, false, ui_idle, settings_nav_callback, settings_controls_callback);
}

//  ----------------------------------------------------------- 
//  --------------------- HOME SCREEN MENU --------------------
//  ----------------------------------------------------------- 
void ui_app_quit(void)
{
  os_sched_exit(-1);
}


// home page defintion
void ui_idle(void)
{

  #define SETTINGS_BUTTON_ENABLED (true)
  nbgl_useCaseHome(
      APPNAME,
      &C_stax_app_near_64px,
      NULL,
      SETTINGS_BUTTON_ENABLED,
      ui_menu_settings,
      ui_app_quit);
}



#endif
