import React, {useState, useEffect} from 'react';
import './CSS/App.css'
import PreviewComponent from './Components/Preview'

const App: React.FC = () => {
  useEffect(() => {
  
  }, [])

  return (
    <>
      <div className={`container-main-window`}>
        <PreviewComponent />
      </div>
      
    </>
  )
}

export default App;
