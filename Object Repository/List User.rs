<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>List User</name>
   <tag></tag>
   <elementGuidId>04b72d6d-641f-40c0-a9c9-e2a161ea8817</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus\&quot;,\n    \&quot;job\&quot;: \&quot;zion resident\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>ec6916bc-84c3-4097-82b4-6eec3f17d9e1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.url}/api/users?page=1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//verify data 0
WS.verifyElementPropertyValue(response, 'data[0].id', '1')
WS.verifyElementPropertyValue(response, 'data[0].email', 'george.bluth@reqres.in')
WS.verifyElementPropertyValue(response, 'data[0].first_name', 'George')
WS.verifyElementPropertyValue(response, 'data[0].last_name', 'Bluth')
WS.verifyElementPropertyValue(response, 'data[0].avatar', 'https://reqres.in/img/faces/1-image.jpg')

//verify data 1
WS.verifyElementPropertyValue(response, 'data[1].id', '2')
WS.verifyElementPropertyValue(response, 'data[1].email', 'janet.weaver@reqres.in')
WS.verifyElementPropertyValue(response, 'data[1].first_name', 'Janet')
WS.verifyElementPropertyValue(response, 'data[1].last_name', 'Weaver')
WS.verifyElementPropertyValue(response, 'data[1].avatar', 'https://reqres.in/img/faces/2-image.jpg')

//verify data 2
WS.verifyElementPropertyValue(response, 'data[2].id', '3')
WS.verifyElementPropertyValue(response, 'data[2].email', 'emma.wong@reqres.in')
WS.verifyElementPropertyValue(response, 'data[2].first_name', 'Emma')
WS.verifyElementPropertyValue(response, 'data[2].last_name', 'Wong')
WS.verifyElementPropertyValue(response, 'data[2].avatar', 'https://reqres.in/img/faces/3-image.jpg')

//verify data 3
WS.verifyElementPropertyValue(response, 'data[3].id', '4')
WS.verifyElementPropertyValue(response, 'data[3].email', 'eve.holt@reqres.in')
WS.verifyElementPropertyValue(response, 'data[3].first_name', 'Eve')
WS.verifyElementPropertyValue(response, 'data[3].last_name', 'Holt')
WS.verifyElementPropertyValue(response, 'data[3].avatar', 'https://reqres.in/img/faces/4-image.jpg')

//verify data 4
WS.verifyElementPropertyValue(response, 'data[4].id', '5')
WS.verifyElementPropertyValue(response, 'data[4].email', 'charles.morris@reqres.in')
WS.verifyElementPropertyValue(response, 'data[4].first_name', 'Charles')
WS.verifyElementPropertyValue(response, 'data[4].last_name', 'Morris')
WS.verifyElementPropertyValue(response, 'data[4].avatar', 'https://reqres.in/img/faces/5-image.jpg')

//verify data 5
WS.verifyElementPropertyValue(response, 'data[5].id', '6')
WS.verifyElementPropertyValue(response, 'data[5].email', 'tracey.ramos@reqres.in')
WS.verifyElementPropertyValue(response, 'data[5].first_name', 'Tracey')
WS.verifyElementPropertyValue(response, 'data[5].last_name', 'Ramos')
WS.verifyElementPropertyValue(response, 'data[5].avatar', 'https://reqres.in/img/faces/6-image.jpg')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
