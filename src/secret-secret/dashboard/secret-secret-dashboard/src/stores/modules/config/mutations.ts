export const setLoading = (state: ConfigState, loading: boolean): void => {
  state.loading = loading;
};

export const setCookiePreference = (
  state: ConfigState,
  preference: boolean
): void => {
  state.cookiePreference = preference;
};
