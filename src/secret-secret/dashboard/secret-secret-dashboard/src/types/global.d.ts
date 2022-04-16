declare interface Secret {
  message: string;
  user_id: string;
  secret_id: string;
  likes: number;
  dislikes: number;
}

// Secret Module
declare interface SecretsState {
  secret: Secret | undefined;
}

// App State
declare interface State {
  secrets: secrets;
}

// Config State
declare interface ConfigState {
  loading: boolean;
  cookiePreference: ?boolean;
}

interface DispatchOutput<T> {
  type: string;
  payload: T;
}
