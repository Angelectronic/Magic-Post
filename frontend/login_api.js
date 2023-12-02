const express = require('express');
const app = express();
const bodyParser = require('body-parser');

app.use(bodyParser.json());

const users = [
  { username: 'user1', password: 'password1' },
  { username: 'user2', password: 'password2' }
];

app.post('/api/login', (req, res) => {
  const { username, password } = req.body;

  const user = users.find(u => u.username === username && u.password === password);

  if (user) {
    res.json({ message: 'Login successful', user });
  } else {
    res.status(401).json({ message: 'Invalid username or password' });
  }
});

app.listen(3000, () => {
  console.log('Server started on port 3000');
});
