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
-- Name: customertable; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customertable (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cemail character(50) NOT NULL,
    cmobile character(50) NOT NULL,
    eid integer NOT NULL,
    dataid integer
);


ALTER TABLE public.customertable OWNER TO postgres;

--
-- Data for Name: customertable; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customertable (cid, cname, cage, cemail, cmobile, eid, dataid) FROM stdin;
110	musta karim	35	m_karim@gmail.com                                 	0805796358                                        	102	5
111	lilian jaiya	43	l_jaiye@gmail.com                                 	0905986359                                        	100	3
112	arthur musa	50	a_musa@gmail.com                                  	0705986378                                        	107	10
113	philip akonjo	41	p_akonjo@gmail.com                                	0905986372                                        	100	2
114	marylene mapa	33	m_mapa@gmail.com                                  	0805986351                                        	120	5
115	oghenero agor	33	o_agor@gmail.com                                  	0705986374                                        	117	11
116	adams bree	50	a_bree@gmail.com                                  	0805986324                                        	102	1
117	okafor mathias	45	o_mathias@gmail.com                               	0805986367                                        	102	10
118	samson adeleke	65	s_adeleke@gmail.com                               	07059863323                                       	117	11
119	lawal tamire	35	l_tamire@gmail.com                                	0905211101                                        	107	5
120	james job	44	j_job@gmail.com                                   	08052117419                                       	100	8
121	matthew jakande	21	m_jackande@gmail.com                              	0705817419                                        	120	2
122	jimila adegboye	20	j_adegboye@gmail.com                              	0804357895                                        	107	5
\.


--
-- Name: customertable customertable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customertable
    ADD CONSTRAINT customertable_pkey PRIMARY KEY (cid);


--
-- PostgreSQL database dump complete
--

