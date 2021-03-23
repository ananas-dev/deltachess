import logo from "./logo.svg";
import "./styles/App.scss";

import ChessBoard from "./components/ChessBoard";
import ControlBox from "./components/ControlBox";

function App() {
  return (
    <div class="container">
      <div class="main">
        <div class="box">
          <ChessBoard />
        </div>
      </div>
      <div>
        <ControlBox />
      </div>
    </div>
  );
}

export default App;
