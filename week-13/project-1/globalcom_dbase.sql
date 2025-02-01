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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    dataid integer NOT NULL,
    datasize character varying(50) NOT NULL,
    dataduration integer NOT NULL,
    dataprice integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation character(50),
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration character varying(50) NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (dataid, datasize, dataduration, dataprice) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION	IKEJA                                             	44
101	2	ACCOUNT	EGBEDA                                            	11
100	3	PACKAGING	AJAH                                              	44
120	4	RESEARCH	V.I                                               	33
97	5	ACCOUNT	MAGODO                                            	22
122	6	OPERATIONS	MILE 2                                            	44
107	7	PACKAGING	KETU                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 months	102
22	B	14 months	97
33	C	16 months	120
44	D	25 months	108
55	E	9 months	107
\.


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
-- Name: customertable customertable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customertable
    ADD CONSTRAINT customertable_pkey PRIMARY KEY (cid);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (dataid);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staffid);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

