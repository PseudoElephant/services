import { ActionContext } from "vuex";

export const setCookiePreference = (
  context: ActionContext<ConfigState, State>,
  preference: boolean
): void => {
  context.commit("setCookiePreference", preference);
};
