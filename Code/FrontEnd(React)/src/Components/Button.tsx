import React from 'react';

interface ButtonProps {
    text: string;
    onClick?: () => void;
    variant?: 'primary' | 'secondary';
}

const Button: React.FC<ButtonProps> = ({ 
    text, 
    onClick, 
    variant = 'primary' 
}) => {
    return (
        <button 
            className={`button ${variant}`}
            onClick={onClick}
        >
            {text}
        </button>
    );
};

export default Button;