export const getSecret = (state: SecretsState): (() => Secret | undefined) => {
  return () => state.secret;
};
