--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staffid integer NOT NULL,
    staffname character(50) NOT NULL,
    dno integer NOT NULL,
    staffsal character(50) NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staffid, staffname, dno, staffsal, age, mobile) FROM stdin;
101	alade                                             	2	250000                                            	33	08023089832
100	mustapha ali                                      	3	175000                                            	32	08063285831
107	alokwe martin                                     	7	380000                                            	48	07090082812
97	dankade aminat                                    	5	550000                                            	40	09023688832
108	josiah joshua                                     	1	120000                                            	30	08053156798
102	mankinde mary                                     	2	450000                                            	55	0808656896
120	adeleke jane                                      	4	200000                                            	38	08024685367
122	osahon mark                                       	6	320000                                            	44	09094146907
104	kuti lawal                                        	1	750000                                            	35	091457952432
117	suleman ajayi                                     	3	800000                                            	50	7030089981
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staffid);


--
-- PostgreSQL database dump complete
--

