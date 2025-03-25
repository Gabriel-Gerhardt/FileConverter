import { event } from "@tauri-apps/api";
import { useState } from 'react';


function Converter({name}) {
    const trueName = name
    const [value, setValue] = useState(0);
    const [updatedName, setUpdatedName] = useState(name);  // Local state for updated name

  const handleClick = () => {
    setValue(value => value + 1);  // Increment value by 1
    if (value %2== 0) {  // After incrementing, check the condition
      setUpdatedName('Clicked');  // Set updatedName to 'new' if value is greater than 1
    }
    else{
        setUpdatedName(trueName)
    }
  };

    return (
        <button type="button" className="convert" onClick={handleClick}>
            {updatedName}
        </button>
    );
}

export default Converter;
