import { useEffect, useState } from "react";
import { io } from "socket.io-client";

const socket = io("http://localhost:3000/ws");

function App() {
    const [board, setBoard] = useState("");

    useEffect(() => {
        socket.on("message", (data) => {
            setBoard(data);
        });
    }, []);

    const sendMove = (move) => {
        socket.emit("message", move);
    };

    return (
        <div>
            <h1>Multiplayer Chess</h1>
            <pre>{board}</pre>
            <button onClick={() => sendMove("e2e4")}>Move e2 to e4</button>
        </div>
    );
}

export default App;
