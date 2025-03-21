import React, { useState } from 'react';

function Input() {
    const [filePath, setFilePath] = useState('');

    const handleFileChange = (event) => {
        const file = event.target.files[0];
        if (file) {
            setFilePath(file.name); // Get the name of the file (you can get the full path, but this is restricted for security reasons)
        }
    };

    return (
        <>
            <label htmlFor="file-input">Selecione um arquivo:</label>
            <input type="file" id="file-input" className="file" onChange={handleFileChange} />
        </>
    );
}

export default Input;
