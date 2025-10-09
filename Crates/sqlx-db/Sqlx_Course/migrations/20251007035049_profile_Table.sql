CREATE TABLE profile (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  user_name VARCHAR(50) NOT NULL,
  full_name VARCHAR(100) NOT NULL
);


INSERT INTO profile (user_name, full_name)
VALUES
    ('Lydia',   'Lydia Fullname'),
    ('Stella',  'Stella Fullname'),
    ('Cate',    'Cate Fullname'),
    ('Braisen', 'Braisen Fullname'),
    ('David',   'David Fullname'),
    ('Kamau',   'Kamau Fullname'),
    ('Vivian',  'Vivian Fullname'),
    ('Faith',   'Faith Fullname'),
    ('Agnes',   'Agnes Fullname'),
    ('Cynthia', 'Cynthia Fullname');
