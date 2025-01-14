import "./style/History.css";
import "./style/Universal.css";
function History() {
    const hist = ["1st record", "2nd record", "3rd record"];
    // const hist = [];

    return (
        <div className="containerHistory">
            <h1>History</h1>
            {hist.length === 0 && <p>No recorded history</p>}
            <ul className="containerHistoryLinks">
                {hist.map((item) => (
                    <li className="noBullet" key={item}>
                        <button className="styleButton">{item}</button>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default History;
