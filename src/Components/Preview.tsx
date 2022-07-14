import React, {useState, useEffect} from 'react';
import '../CSS/Preview.css'

const App: React.FC = () => {
  return (
    <>
      <input type='text' placeholder='State' className='stateInput'></input>
      <input type='text' placeholder='Details' className='detailsInput'></input>
    </>
  )
}

export default App;
