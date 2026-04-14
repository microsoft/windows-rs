#ifndef _SDV_
// general purpose save 
//-----------------------
#define	__sdv_save_request(r)
// general purpose retrieve 
//-----------------------
#define	__sdv_retrieve_request(r)

// NDIS AdapterContext save 
//-----------------------
#define __sdv_save_adapter_context(c)

#else
// general purpose save 
//-----------------------
//#define	__sdv_save sdv_save
//void sdv_save(void *r){;}
// general purpose retrieve macros
//-----------------------
//#define	__sdv_retrieve sdv_retrieve
//void sdv_retrieve(void *r){;}
#endif
