export const defaltState: SecretsState = {
  secret: {
    message: "Dummy Message",
    user_id: "",
    secret_id: "",
    likes: 0,
    dislikes: 0,
  },
};

export default {
  ...defaltState,
};
