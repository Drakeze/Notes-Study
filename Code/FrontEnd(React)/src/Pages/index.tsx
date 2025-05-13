import React, { useState } from 'react';
import Button from '../Components/Button';

const HomePage: React.FC = () => {
    const [count, setCount] = useState<number>(0);

    const handleClick = () => {
        setCount(prev => prev + 1);
    };

    return (
        <div className="container">
            <h1>Next.js with TypeScript Practice</h1>
            <p>Count: {count}</p>
            <Button 
                text="Increment" 
                onClick={handleClick} 
                variant="primary"
            />
        </div>
    );
};

export default HomePage;