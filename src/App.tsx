import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri';

let bool = true;
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
          Learn React
        </a>
        <iframe src="tune://foo.bar" title="thingy"/>
        <div id="res"/>
        <div id="err"/>
      </header>
      <button onClick={() =>
        invoke('my_custom_command', {number: 42})
          .then((res:any) => {
            // <audio id="audio-element" src="tune://razor.flc" autoPlay={true}/>
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
            const a = document.getElementById('audio-element');
            if (!a) return;
            const ae = a as HTMLAudioElement;
            ae.src=bool ? "tune://foo.flac" : "tune://bar.mp3";
            bool = !bool;
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
