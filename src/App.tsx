import { useEffect, useState } from 'react';
import './App.css';

function App() {
    const [text, setText] = useState<string>("");


    useEffect(() => {
        const fetchText = async () => {
            const response = await fetch('/api');
            const data = await response.text();
            console.info(data);
            setText(data);
        };
        fetchText();
    }, []);

    return (
        <div className="App">
            <header className="App-header">
                <div>
                    {text}
                </div>
            </header>
        </div>
    );
}

export default App;
