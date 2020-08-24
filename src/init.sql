drop table if exists people;
create table people (
	id uuid primary key,
	fn text,
	mn text,
	ln text,
	bd date
);
insert into people values 
('16c5fb08-d2bb-11ea-a425-77874abfec0e', 'Bob','The','Builder','1/1/2000'),
('bc82f0f0-e503-11ea-b322-1fb9e78e6134', 'Jeff','Gregory','Rogers','1/1/1881'),
('42d357c0-e523-11ea-9b76-7b68a453235e', 'Sam','Samual','Smith','1/1/1991');

select * from people;