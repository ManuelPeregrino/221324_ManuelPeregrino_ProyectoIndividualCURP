import React, { useState } from 'react';
import axios from 'axios';

const CURPForm = () => {
  const [formData, setFormData] = useState({
    first_name: '',
    father_surname: '',
    mother_surname: '',
    birth_date: '', // Formato: YYYY-MM-DD
    gender: '', // 'H' para hombre, 'M' para mujer
    birth_state: '',
  });
  const [curp, setCurp] = useState('');

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prevFormData) => ({
      ...prevFormData,
      [name]: value
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post('http://localhost:8080/generate_curp', formData);
      setCurp(response.data.curp);
    } catch (error) {
      console.error('There was an error generating the CURP:', error);
      // Manejar el error apropiadamente
    }
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <input type="text" name="first_name" value={formData.first_name} onChange={handleChange} placeholder="Nombre(s)" required />
        <input type="text" name="father_surname" value={formData.father_surname} onChange={handleChange} placeholder="Apellido Paterno" required />
        <input type="text" name="mother_surname" value={formData.mother_surname} onChange={handleChange} placeholder="Apellido Materno" required />
        <input type="date" name="birth_date" value={formData.birth_date} onChange={handleChange} required />
        <select name="gender" value={formData.gender} onChange={handleChange} required>
          <option value="">Género</option>
          <option value="H">Hombre</option>
          <option value="M">Mujer</option>
        </select>
        <input type="text" name="birth_state" value={formData.birth_state} onChange={handleChange} placeholder="Entidad de Nacimiento" required />
        <button type="submit">Generar CURP</button>
      </form>
      {curp && <textarea value={curp} readOnly />}
    </div>
  );
};

export default CURPForm;
