import { useEffect, useState, useRef } from "react";
import type { MouseEvent } from "react"; // <-- Importación exclusiva de tipo
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import "./App.css";

// Assets
import pikachuIdle from "./assets/hero.png"; // Usamos el Pikachu que ya tienes
// import pikachuSleep from "./assets/pikachu_sleep.gif"; // Sprite temporal para dormir

// Interface matching the updated Rust SystemResources struct
interface SystemResources {
  cpu_usage_percentage: number;
  used_memory_mb: number;      // Updated: Now used memory
  total_memory_mb: number;
}

// Logical states for the pet
type PetState = 'Idle' | 'Sleeping' | 'Stressed';

function App() {
  const [resources, setResources] = useState<SystemResources | null>(null);
  const [petState, setPetState] = useState<PetState>('Idle');
  const petRef = useRef<HTMLDivElement>(null); // Reference to the pet element

  // Function to request hardware data from the Rust backend
  const fetchSystemResources = async () => {
    try {
      const data = await invoke<SystemResources>("get_system_resources");
      setResources(data);
    } catch (error) {
      console.error("Failed to fetch system resources:", error);
    }
  };

  // NEW: Lógica para determinar el estado de la mascota basado en la hora
  const updatePetState = () => {
    const hour = new Date().getHours();
    
    // Logic: Sleep after 10 PM (22:00) or before 7 AM
    if (hour >= 22 || hour < 7) {
      setPetState('Sleeping');
    } else {
      setPetState('Idle');
    }
  };

  useEffect(() => {
    // Initial fetch
    fetchSystemResources();
    updatePetState();

    // Set an interval to poll resources and check time every 5 seconds
    const intervalId = setInterval(() => {
      fetchSystemResources();
      updatePetState();
    }, 5000);

    return () => clearInterval(intervalId);
  }, []);

  // Native Dragging Logic (Tauri API)
  const handleMouseDown = async (e: MouseEvent<HTMLDivElement>) => {
    // Solo queremos que el botón izquierdo (button 0) active el arrastre
    if (e.button === 0) {
      // Esta función de Tauri inicia una operación de arrastre nativa de la ventana.
      // Es muy eficiente ya que delega el trabajo al motor del sistema operativo.
      await appWindow.startDragging();
    }
  };

  // Determinar qué sprite mostrar basándose en el estado
  const currentSprite = petState === 'Sleeping' ? pikachuIdle : pikachuIdle; // Aquí usarías pikachuSleep si tienes el gif

  return (
    // The main container fills the entire transparent window (width/height from CSS)
    <div className="pet-container">
      
      {/* El Sprite de la Mascota: 
        MouseDown dispara handleMouseDown, que le dice a Tauri que arrastre la ventana.
      */}
      <div 
        ref={petRef}
        className="pet-sprite" 
        onMouseDown={handleMouseDown} // CRÍTICO para el arrastre
        style={{ cursor: 'grab' }}
      >
        <img 
          src={currentSprite} // Cambia automáticamente según el estado
          alt={`My Pikachu is ${petState}`} 
          style={{ width: '120px', imageRendering: 'pixelated' }} 
        />
        
        {/*
          Debug UI inside the sprite's layout to keep it aligned.
          Using the debug-text class from CSS.
        */}
        <div className="debug-text">
          <p>CPU: {resources?.cpu_usage_percentage.toFixed(1)}%</p>
          <p>RAM: {resources?.used_memory_mb}MB / {resources?.total_memory_mb}MB</p>
          {/* Opcional: mostrar estado actual */}
          {/* <p>Status: {petState}</p> */}
        </div>
      </div>
      
    </div>
  );
}

export default App;