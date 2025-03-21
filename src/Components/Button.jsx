
function Converter({name}) {

    return (
        <button type="button" className="convert" onClick={() => console.log('hello')}>
            {name}
        </button>
    );
}

export default Converter;
