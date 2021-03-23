import { createState } from "solid-js";

const ControlBox = () => {
  const [state, setState] = createState({
    users: [
      {
        name: "ananas-dev",
        type: "humain",
        helo: 1300,
      },
      {
        name: "stockfish",
        type: "bot",
      },
    ],
  });
  return (
    <div>
      {state.users.map((user) => (
        <div class="name">{user.name}</div>
      ))}
    </div>
  );
};

export default ControlBox;
