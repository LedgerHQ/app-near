#ifdef HAVE_BAGL
#include "os.h"
#include "menu.h"
#include "globals.h"


//////////////////////////////////////////////////////////////////////
const char *settings_submenu_getter(unsigned int idx);
void settings_submenu_selector(unsigned int idx);

//////////////////////////////////////////////////////////////////////////////////////
// Enable contract data submenu:

void blind_sign_enabled_data_change(blind_sign_switch_state_t new_blind_sign)
{
  PRINTF("current value of blind sign: %d\n", blind_sign_enabled);
  uint8_t value;
  switch (new_blind_sign) {
      case BLSGN_OFF_STATE:
      case BLSGN_ON_STATE:
          value = (uint8_t) new_blind_sign;
          if (value != blind_sign_enabled) {
            nvm_write((void *)&N_storage.blind_sign_enabled, &value, sizeof(value));
            blind_sign_enabled = value;
          }
          break;
  }
}

const char *const blind_sign_enabled_data_getter_values[] = {
    "Enable",
    "Disable",
    "Back"};

const char *blind_sign_enabled_data_getter(unsigned int idx)
{
  if (idx < ARRAYLEN(blind_sign_enabled_data_getter_values))
  {
    return blind_sign_enabled_data_getter_values[idx];
  }
  return NULL;
}

void blind_sign_enabled_data_selector(unsigned int idx)
{
  switch (idx)
  {
  case 0:
    blind_sign_enabled_data_change(BLSGN_ON_STATE);
    break;
  case 1:
    blind_sign_enabled_data_change(BLSGN_OFF_STATE);
    break;
  default:
    break;
  }
  ux_menulist_init(0, settings_submenu_getter, settings_submenu_selector);
}

//////////////////////////////////////////////////////////////////////////////////////
// Settings menu:
const u_int8_t BLIND_SIG_IDX = 0;

const char *const settings_submenu_getter_values[] = {
    "Blind Sign",
    "Back",
};

const char *const blind_signature_title_values[] = {
    "Blind Sign (Off)",
    "Blind Sign (On)",
};

const char *settings_submenu_getter(unsigned int idx)
{
  if (idx < ARRAYLEN(settings_submenu_getter_values))
  {
    if (idx == BLIND_SIG_IDX) {
        return blind_signature_title_values[blind_sign_enabled];
    }
    return settings_submenu_getter_values[idx];
  }
  return NULL;
}

void settings_submenu_selector(unsigned int idx)
{
  switch (idx)
  {
  case 0:
    ux_menulist_init_select(0, blind_sign_enabled_data_getter, blind_sign_enabled_data_selector, N_storage.blind_sign_enabled);
    break;
  default:
    ui_idle();
  }
}

//////////////////////////////////////////////////////////////////////
UX_STEP_NOCB(
    ux_idle_flow_1_step,
    pnn,
    {&C_icon_near,
     "Use wallet to", "view accounts"});
UX_STEP_VALID(
    ux_idle_flow_2_step,
    pb,
    ux_menulist_init(0, settings_submenu_getter, settings_submenu_selector),
    {
        &C_icon_coggle,
        "Settings",
    });
UX_STEP_NOCB(
    ux_idle_flow_3_step,
    bn,
    {
        "Version",
        APPVERSION,
    });
UX_STEP_VALID(
    ux_idle_flow_4_step,
    pb,
    os_sched_exit(-1),
    {
        &C_icon_dashboard_x,
        "Quit",
    });
UX_FLOW(ux_idle_flow,
        &ux_idle_flow_1_step,
        &ux_idle_flow_2_step,
        &ux_idle_flow_3_step,
        &ux_idle_flow_4_step,
        FLOW_LOOP);

void ui_idle(void)
{
  // reserve a display stack slot if none yet
  if (G_ux.stack_count == 0)
  {
    ux_stack_push();
  }
  ux_flow_init(0, ux_idle_flow, NULL);
}

#endif
