/* eslint-disable @typescript-eslint/no-empty-function */

import { Action, ActionContext } from "vuex";

// Its only declared to trigger our sagas.
export const fetchSecret: Action<SecretsState, State> = () => {};
export const createSecret: Action<SecretsState, State> = () => {};

export const setSecret = (
  context: ActionContext<SecretsState, State>,
  secret: Secret
): void => {
  context.commit("setSecret", secret);
};
