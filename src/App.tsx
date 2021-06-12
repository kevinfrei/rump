import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to magically reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          {/* <audio src="Razor.flac" autoPlay={true}/> */ }
          Learn React
        </a>
        <div id="res"/>
        <div id="err"/>
      </header>
      <button onClick={() =>
        invoke('my_custom_command', {number: 42})
          .then((res:any) => {
            const r = document.getElementById('res');
            if (!r) return;
            let str = "Found Res. ";
            if (res.message) {
              str += res.message;
            }
            if (res.other_val) {
              str += res.other_val;
            }
            r.innerText += "\n" + str;
          })
          .catch((e) => {
            const el = document.getElementById('err');
            if (el) {
              el.innerText = e.toString();
            }
          })
      }>Click Me</button>
    </div>
  );
}

export default App;
