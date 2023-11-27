import React, { useState } from 'react';

const FormBox = () => {
  const [showForm, setShowForm] = useState(false);

  const toggleForm = () => {
    setShowForm(!showForm);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    // Xử lý dữ liệu khi người dùng gửi form
    console.log('Form submitted!');
  };

  return (
    <div>
      <div onClick={toggleForm} style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100px', width: '200px', border: '1px solid #000', cursor: 'pointer' }}>
        Click here to fill out the form
      </div>
      {showForm && (
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', marginTop: '20px' }}>
          <label>
            Name:
            <input type="text" />
          </label>
          <label>
            Address:
            <input type="text" />
          </label>
          <label>
            Phone Number:
            <input type="text" />
          </label>
          <button type="submit">Submit</button>
        </form>
      )}
    </div>
  );
};

export default FormBox;
